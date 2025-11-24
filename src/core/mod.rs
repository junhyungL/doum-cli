// 핵심 비즈니스 로직 모듈

pub mod ask;
pub mod auto_mode;
pub mod suggest;

pub use ask::handle_ask;
pub use auto_mode::select_and_execute;
pub use suggest::handle_suggest;
