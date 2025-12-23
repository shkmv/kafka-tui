//! Low-level FFI bindings for rdkafka admin operations not exposed by the safe API.
//!
//! This module contains unsafe code for admin operations that are not available
//! through rdkafka's safe Rust API, specifically the DeleteRecords operation.
//!
//! # Safety
//!
//! Functions in this module must be called with valid pointers obtained from
//! rdkafka client objects. The caller is responsible for ensuring the underlying
//! Kafka client remains valid for the duration of these calls.

use std::ffi::CStr;

use rdkafka::bindings as rdsys;
use rdkafka::TopicPartitionList;

use crate::error::{AppError, AppResult};

/// Delete records from topic partitions up to the specified offsets.
///
/// This function uses the raw rdkafka FFI to perform the DeleteRecords admin operation,
/// which is not exposed through the safe rdkafka API.
///
/// # Arguments
///
/// * `client_ptr` - Raw pointer to the rd_kafka_t client, cast as usize for Send safety
/// * `tpl` - Topic partition list with offsets indicating where to delete up to
/// * `timeout_ms` - Operation timeout in milliseconds
///
/// # Safety
///
/// This function is safe to call if:
/// - `client_ptr` is a valid pointer to an rd_kafka_t obtained from a live consumer
/// - The consumer must remain valid for the duration of this call
/// - `tpl` must contain valid topic-partition-offset entries
///
/// The function handles cleanup of all allocated rdkafka resources on all error paths.
pub fn delete_records(
    client_ptr: usize,
    tpl: TopicPartitionList,
    timeout_ms: i32,
) -> AppResult<()> {
    // SAFETY: caller guarantees client_ptr is valid for the duration of this call
    unsafe { delete_records_inner(client_ptr as *mut rdsys::rd_kafka_t, tpl, timeout_ms) }
}

/// Inner implementation of delete_records with the actual unsafe operations.
unsafe fn delete_records_inner(
    client_ptr: *mut rdsys::rd_kafka_t,
    tpl: TopicPartitionList,
    timeout_ms: i32,
) -> AppResult<()> {
    // Create admin result queue
    let queue = unsafe { rdsys::rd_kafka_queue_new(client_ptr) };
    if queue.is_null() {
        return Err(AppError::Kafka("Failed to create admin result queue".into()));
    }

    // Create admin options for DeleteRecords operation
    let mut errstr = [0i8; 512];
    let opts = unsafe {
        rdsys::rd_kafka_AdminOptions_new(
            client_ptr,
            rdsys::rd_kafka_admin_op_t::RD_KAFKA_ADMIN_OP_DELETERECORDS,
        )
    };
    if opts.is_null() {
        unsafe { rdsys::rd_kafka_queue_destroy(queue) };
        return Err(AppError::Kafka("Failed to create admin options".into()));
    }

    // Set request timeout
    let timeout_result = unsafe {
        rdsys::rd_kafka_AdminOptions_set_request_timeout(
            opts,
            timeout_ms,
            errstr.as_mut_ptr(),
            errstr.len(),
        )
    };
    if timeout_result != rdsys::rd_kafka_resp_err_t::RD_KAFKA_RESP_ERR_NO_ERROR {
        let msg = unsafe { CStr::from_ptr(errstr.as_ptr()) }.to_string_lossy().into_owned();
        unsafe {
            rdsys::rd_kafka_AdminOptions_destroy(opts);
            rdsys::rd_kafka_queue_destroy(queue);
        }
        return Err(AppError::Kafka(format!("Failed to set timeout: {}", msg)));
    }

    // Create DeleteRecords request
    let del_records = unsafe { rdsys::rd_kafka_DeleteRecords_new(tpl.ptr()) };
    if del_records.is_null() {
        unsafe {
            rdsys::rd_kafka_AdminOptions_destroy(opts);
            rdsys::rd_kafka_queue_destroy(queue);
        }
        return Err(AppError::Kafka("Failed to create DeleteRecords request".into()));
    }

    // Issue the DeleteRecords request
    let mut del_records_arr = [del_records];
    unsafe {
        rdsys::rd_kafka_DeleteRecords(
            client_ptr,
            del_records_arr.as_mut_ptr(),
            del_records_arr.len(),
            opts,
            queue,
        );
    }

    // Cleanup request resources (the request has been queued)
    unsafe {
        rdsys::rd_kafka_DeleteRecords_destroy(del_records);
        rdsys::rd_kafka_AdminOptions_destroy(opts);
    }

    // Wait for and process the result
    let event = unsafe { rdsys::rd_kafka_queue_poll(queue, timeout_ms) };
    if event.is_null() {
        unsafe { rdsys::rd_kafka_queue_destroy(queue) };
        return Err(AppError::Kafka("DeleteRecords timed out".into()));
    }

    // Check for operation-level error
    let err = unsafe { rdsys::rd_kafka_event_error(event) };
    if err != rdsys::rd_kafka_resp_err_t::RD_KAFKA_RESP_ERR_NO_ERROR {
        let c_msg = unsafe { rdsys::rd_kafka_event_error_string(event) };
        let msg = if c_msg.is_null() {
            "DeleteRecords failed".to_string()
        } else {
            unsafe { CStr::from_ptr(c_msg) }.to_string_lossy().into_owned()
        };
        unsafe {
            rdsys::rd_kafka_event_destroy(event);
            rdsys::rd_kafka_queue_destroy(queue);
        }
        return Err(AppError::Kafka(msg));
    }

    // Get the result
    let result = unsafe { rdsys::rd_kafka_event_DeleteRecords_result(event) };
    if result.is_null() {
        unsafe {
            rdsys::rd_kafka_event_destroy(event);
            rdsys::rd_kafka_queue_destroy(queue);
        }
        return Err(AppError::Kafka("DeleteRecords returned unexpected result".into()));
    }

    // Check per-partition results
    if let Err(e) = unsafe { check_partition_results(result) }? {
        unsafe {
            rdsys::rd_kafka_event_destroy(event);
            rdsys::rd_kafka_queue_destroy(queue);
        }
        return Err(e);
    }

    // Cleanup
    unsafe {
        rdsys::rd_kafka_event_destroy(event);
        rdsys::rd_kafka_queue_destroy(queue);
    }
    Ok(())
}

/// Check the per-partition results from a DeleteRecords operation.
unsafe fn check_partition_results(
    result: *const rdsys::rd_kafka_DeleteRecords_result_t,
) -> AppResult<Result<(), AppError>> {
    let offsets = unsafe { rdsys::rd_kafka_DeleteRecords_result_offsets(result) };
    if offsets.is_null() {
        return Err(AppError::Kafka("DeleteRecords returned no offsets".into()));
    }

    let offsets = unsafe { &*offsets };
    for i in 0..(offsets.cnt as isize) {
        let elem = unsafe { &*offsets.elems.offset(i) };
        if elem.err != rdsys::rd_kafka_resp_err_t::RD_KAFKA_RESP_ERR_NO_ERROR {
            let topic_name = if elem.topic.is_null() {
                "<unknown>".to_string()
            } else {
                unsafe { CStr::from_ptr(elem.topic) }.to_string_lossy().into_owned()
            };
            return Ok(Err(AppError::Kafka(format!(
                "DeleteRecords failed for {}[{}]: {:?}",
                topic_name, elem.partition, elem.err
            ))));
        }
    }

    Ok(Ok(()))
}

#[cfg(test)]
mod tests {
    // Note: These tests would require a running Kafka cluster and are therefore
    // integration tests. Unit tests for this module are limited due to the FFI nature.
}
