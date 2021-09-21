pub use self::available_documents_ids::AvailableDocumentsIds;
pub use self::clear_documents::ClearDocuments;
pub use self::delete_documents::DeleteDocuments;
pub use self::facets::Facets;
pub use self::index_documents::{DocumentAdditionResult, IndexDocuments, IndexDocumentsMethod};
pub use self::settings::{Setting, Settings};
pub use self::update_builder::UpdateBuilder;
pub use self::update_step::UpdateIndexingStep;
pub use self::word_prefix_docids::WordPrefixDocids;
pub use self::word_prefix_pair_proximity_docids::WordPrefixPairProximityDocids;
pub use self::words_level_positions::WordsLevelPositions;
pub use self::words_prefixes_fst::WordsPrefixesFst;

mod available_documents_ids;
mod clear_documents;
mod delete_documents;
mod facets;
mod index_documents;
mod settings;
mod update_builder;
mod update_step;
mod word_prefix_docids;
mod word_prefix_pair_proximity_docids;
mod words_level_positions;
mod words_prefixes_fst;
