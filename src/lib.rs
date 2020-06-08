//! Asset management resource locator defined as a uri-ish structure:
//!
//! scheme://level/name/department/subcontext/type?version=value#type[/resource/path]
//! eg
//! asset://dev01/thanos/model/hi/maya_model?version=current#main
//! instance:/dev01.rd.9999/thanos1/anim/hi/alembic_cache?version=current#main
//!
//! Uri-ish because we leave out the authority (ie no server:port )
//! valid schemes:
//! - asset
//! - instance
//! - render
//! - plate
//!
//! valid name, dept, subcontext, type:
//! char (a-z|0-9_)+
pub mod assetmodel;
pub mod errors;
pub mod level;
pub mod parse;
pub mod scheme;
pub mod version;
