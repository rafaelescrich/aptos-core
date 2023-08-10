// Copyright © Aptos Foundation

use aptos_metrics_core::{register_int_counter, IntCounter};
use once_cell::sync::Lazy;

// OVERALL METRICS

/// Number of times a given processor has been invoked
pub static PARSER_INVOCATIONS_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_processor_invocation_count",
        "Number of times a given processor has been invoked",
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has raised an error
pub static PARSER_ERRORS_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_processor_errors",
        "Number of times the NFT Metadata Crawler Parser has raised an error",
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has completed successfully
pub static PARSER_SUCCESSES_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_processor_success_count",
        "Number of times a given processor has completed successfully",
    )
    .unwrap()
});

// DATABASE METRICS

/// Number of times the connection pool has timed out when trying to get a connection
pub static UNABLE_TO_GET_CONNECTION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_connection_pool_err",
        "Number of times the connection pool has timed out when trying to get a connection"
    )
    .unwrap()
});

/// Number of times the connection pool got a connection
pub static GOT_CONNECTION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_connection_pool_ok",
        "Number of times the connection pool got a connection"
    )
    .unwrap()
});

// DEDUPLICATION METRICS

/// Number of times the NFT Metadata Crawler Parser has found a duplicate token URI
pub static DUPLICATE_TOKEN_URI_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_duplicate_token_uri_count",
        "Number of times the NFT Metadata Crawler Parser has found a duplicate token URI"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has found a duplicate raw image URI
pub static DUPLICATE_RAW_IMAGE_URI_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_duplicate_raw_image_uri_count",
        "Number of times the NFT Metadata Crawler Parser has found a duplicate raw image URI"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has found a duplicate raw animation URI
pub static DUPLICATE_RAW_ANIMATION_URI_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_duplicate_raw_animation_uri_count",
        "Number of times the NFT Metadata Crawler Parser has found a duplicate raw animation URI"
    )
    .unwrap()
});

// URI PARSER METRICS

/// Number of times the NFT Metadata Crawler Parser has invocated the URI Parser
pub static PARSE_URI_INVOCATION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_uri_invocation_count",
        "Number of times the NFT Metadata Crawler Parser has invocated the URI Parser"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has been unable to parse a URI to use the dedicated IPFS gateway
pub static UNABLE_TO_PARSE_URI_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_unable_to_parse_uri_count",
        "Number of times the NFT Metadata Crawler Parser has been unable to parse a URI to use the dedicated IPFS gateway"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has been able to parse a URI to use the dedicated IPFS gateway
pub static SUCCESSFULLY_PARSED_URI_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_uri_count",
        "Number of times the NFT Metadata Crawler Parser has been able to parse a URI to use the dedicated IPFS gateway"
    )
    .unwrap()
});

// JSON PARSER METRICS

/// Number of times the NFT Metadata Crawler has invocated the JSON Parser
pub static PARSE_JSON_INVOCATION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_json_invocation_count",
        "Number of times the NFT Metadata Crawler Parser has invocated the JSON Parser"
    )
    .unwrap()
});

/// Number of times the JSON Parser has been unable to parse a JSON because it was too large
pub static PARSE_JSON_FILE_TOO_LARGE_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_json_file_too_large_count",
        "Number of times the JSON Parser has been unable to parse a JSON because it was too large"
    )
    .unwrap()
});

/// Number of times the JSON Parser has been unable to parse a JSON because an image was found instead
pub static PARSE_JSON_FILE_FOUND_IMAGE_INSTEAD: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_json_file_found_image_instead",
        "Number of times the JSON Parser has been unable to parse a JSON because it found an image instead"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has been unable to parse a JSON
pub static UNABLE_TO_PARSE_JSON_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_unable_to_parse_json_count",
        "Number of times the NFT Metadata Crawler Parser has been unable to parse a JSON"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has been able to parse a JSON
pub static SUCCESSFULLY_PARSED_JSON_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_json_count",
        "Number of times the NFT Metadata Crawler Parser has been able to parse a JSON"
    )
    .unwrap()
});

// IMAGE OPTIMIZER METRICS

/// Number of times the NFT Metadata Crawler Parser has invocated the Image Optimizer
pub static OPTIMIZE_IMAGE_INVOCATION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_optimize_image_invocation_count",
        "Number of times the NFT Metadata Crawler Parser has invocated the Image Optimizer"
    )
    .unwrap()
});

/// Number of times the Image Optimizer has been unable to optimize an image because it was too large
pub static OPTIMIZE_IMAGE_FILE_TOO_LARGE_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_optimize_image_file_too_large_count",
        "Number of times the Image Optimizer has been unable to optimize an image because it was too large"
    )
    .unwrap()
});

/// Number of times the Image Optimizer has been unable to optimize an image
pub static UNABLE_TO_OPTIMIZE_IMAGE_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_unable_to_optimize_image_count",
        "Number of times the Image Optimizer has been unable to optimize an image"
    )
    .unwrap()
});

/// Number of times the Image Optimizer has been able to optimize an image
pub static SUCCESSFULLY_OPTIMIZED_IMAGE_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_optimize_image_count",
        "Number of times the Image Optimizer has been able to optimize an image"
    )
    .unwrap()
});

/// Number of times the Image Optimizer has been unable to optimize an animation
pub static UNABLE_TO_OPTIMIZE_ANIMATION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_unable_to_optimize_animation_count",
        "Number of times the Image Optimizer has been unable to optimize an animation"
    )
    .unwrap()
});

/// Number of times the Image Optimizer has been able to optimize an animation
pub static SUCCESSFULLY_OPTIMIZED_ANIMATION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_optimize_animation_count",
        "Number of times the Image Optimizer has been able to optimize an animation"
    )
    .unwrap()
});

// GCS METRICS

/// Number of times the NFT Metadata Crawler Parser has been unable to write to GCS
pub static UNABLE_TO_WRITE_TO_GCS_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_unable_to_write_to_gcs_count",
        "Number of times the NFT Metadata Crawler Parser has been unable to write to GCS"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has been able to write to GCS
pub static SUCCESSFULLY_WRITTEN_TO_GCS_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_write_to_gcs_count",
        "Number of times the NFT Metadata Crawler Parser has been able to write to GCS"
    )
    .unwrap()
});
