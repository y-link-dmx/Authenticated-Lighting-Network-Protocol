#[macro_export]
macro_rules! assert_latency {
    ($val:expr, $max:expr) => {
        let v = $val;
        assert!(
            v <= $max,
            "latency {}ms exceeds threshold {}ms",
            v,
            $max
        );
    };
}

#[macro_export]
macro_rules! assert_fps_min {
    ($val:expr, $min:expr) => {
        let v = $val;
        assert!(
            v >= $min,
            "fps {} below minimum {}",
            v,
            $min
        );
    };
}

#[macro_export]
macro_rules! assert_session_state {
    ($session:expr, $expected:pat) => {
        match $session.state() {
            $expected => {}
            other => panic!("session state mismatch: {:?}", other),
        }
    };
}

#[macro_export]
macro_rules! assert_frame_delivery_ratio {
    ($delivered:expr, $sent:expr, $min_ratio:expr) => {
        let d = $delivered as f64;
        let s = $sent as f64;
        let ratio = if s == 0.0 { 1.0 } else { d / s };
        assert!(
            ratio >= $min_ratio,
            "delivery ratio {:.2} below {:.2}",
            ratio,
            $min_ratio
        );
    };
}
