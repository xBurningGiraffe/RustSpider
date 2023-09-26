use governor::RateLimiter;
use governor::Quota;
use governor::state::NotKeyed;
use governor::state::InMemoryState;
use governor::clock::DefaultClock;
use std::sync::Arc;
use std::num::NonZeroU32;


pub type SharedRateLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>;

pub fn create_rate_limiter(rate: NonZeroU32) -> SharedRateLimiter {
    Arc::new(RateLimiter::direct(Quota::per_second(rate)))
}
