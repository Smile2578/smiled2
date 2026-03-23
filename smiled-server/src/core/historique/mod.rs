/// Historique module — tooth event timeline.
///
/// The timeline endpoint is wired through `schema_dentaire::router()` at
/// `GET /patients/:id/dent/:fdi/timeline`. This module re-exports the
/// query so it can be used independently if needed by other modules.
pub use crate::core::schema_dentaire::queries::get_tooth_timeline;
