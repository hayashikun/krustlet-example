extern crate serde;
extern crate wascc_actor as actor;
extern crate wascc_codec;

use actor::prelude::*;
use serde::Serialize;

actor_handlers! {
    codec::core::OP_HEALTH_REQUEST => health,
    codec::http::OP_HANDLE_REQUEST => handle
}

fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
    Ok(())
}

fn handle(request: codec::http::Request) -> HandlerResult<codec::http::Response> {
    let path = request.path.trim_start_matches("/");

    let response = match path {
        "is_prime" => handle_is_prime(request),
        "prime_less_than" => handle_prime_less_than(request),
        _ => codec::http::Response::bad_request()
    };

    Ok(response)
}

fn handle_is_prime(request: codec::http::Request) -> codec::http::Response {
    let number: usize = request.query_string.parse().unwrap_or(0);
    if number < 1 {
        return codec::http::Response::bad_request();
    }
    let payload = IsPrimeResponse {
        is_prime: is_prime(number)
    };
    codec::http::Response::json(payload, 200, "OK")
}

fn handle_prime_less_than(request: codec::http::Request) -> codec::http::Response {
    let number: usize = request.query_string.parse().unwrap_or(0);
    if number < 1 {
        return codec::http::Response::bad_request();
    }
    let payload = PrimeLessThanResponse {
        prime: sieve(number)
    };
    codec::http::Response::json(payload, 200, "OK")
}

fn is_prime(n: usize) -> bool {
    sieve(n).last().unwrap_or(&0) == &n
}

fn sieve(n: usize) -> Vec<usize> {
    let mut prime_list = Vec::with_capacity(n + 1);
    for _ in 0..prime_list.capacity() { prime_list.push(true); }
    prime_list[0] = false; // 0 is not prime
    prime_list[1] = false; // 1 is not prime

    for i in 2..=((n as f32).sqrt() as usize) {
        if prime_list[i] {
            let mut j = 2;
            while i * j <= n {
                prime_list[i * j] = false;
                j += 1;
            }
        }
    }
    return prime_list.iter().enumerate().filter(|&(_, &b)| b).map(|(i, _)| i).collect();
}

#[cfg(test)]
mod tests {
    use crate::{is_prime, sieve};

    #[test]
    fn sieve_test() {
        assert_eq!(sieve(10), vec![2, 3, 5, 7]);
        assert_eq!(sieve(11), vec![2, 3, 5, 7, 11]);
        assert!(!is_prime(10));
        assert!(is_prime(11));
    }
}

#[derive(Serialize)]
struct IsPrimeResponse {
    is_prime: bool
}

#[derive(Serialize)]
struct PrimeLessThanResponse {
    prime: Vec<usize>
}


