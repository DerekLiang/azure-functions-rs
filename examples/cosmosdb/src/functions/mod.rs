// WARNING: This file is regenerated by the `cargo func new` command.

mod create_document;
mod log_documents;
mod query_documents;
mod read_document;

// Export the Azure Functions here.
azure_functions::export! {
    create_document::create_document,
    log_documents::log_documents,
    query_documents::query_documents,
    read_document::read_document,
}
