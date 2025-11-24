// 핵심 비즈니스 로직 모듈

pub mod ask;
pub mod suggest;
pub mod auto_mode;

pub use ask::handle_ask;
pub use suggest::handle_suggest;
pub use auto_mode::select_and_execute;
