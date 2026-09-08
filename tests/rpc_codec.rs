use rmpv::Value;

fn encode_call(method: &str, token: Option<&str>, args: &[Value]) -> Vec<u8> {
    let mut arr = vec![Value::from(method)];
    if let Some(t) = token {
        arr.push(Value::from(t));
    }
    arr.extend(args.iter().cloned());
    let mut buf = Vec::new();
    rmpv::encode::write_value(&mut buf, &Value::Array(arr)).unwrap();
    buf
}

#[test]
fn login_roundtrip() {
    let bytes = encode_call(
        "auth.login",
        None,
        &[Value::from("msf"), Value::from("secret")],
    );
    let v = rmpv::decode::read_value(&mut &bytes[..]).unwrap();
    let arr = v.as_array().unwrap();
    assert_eq!(arr[0].as_str().unwrap(), "auth.login");
    assert_eq!(arr[1].as_str().unwrap(), "msf");
    assert_eq!(arr[2].as_str().unwrap(), "secret");
}

#[test]
fn token_inserted() {
    let bytes = encode_call("core.version", Some("tok123"), &[]);
    let v = rmpv::decode::read_value(&mut &bytes[..]).unwrap();
    let arr = v.as_array().unwrap();
    assert_eq!(arr[0].as_str().unwrap(), "core.version");
    assert_eq!(arr[1].as_str().unwrap(), "tok123");
}

#[test]
fn error_map() {
    let v = Value::Map(vec![
        (Value::from("error"), Value::from(true)),
        (Value::from("error_message"), Value::from("bad token")),
    ]);
    let map: Vec<(String, Value)> = v
        .as_map()
        .unwrap()
        .iter()
        .filter_map(|(k, val)| k.as_str().map(|s| (s.to_string(), val.clone())))
        .collect();
    let err = map.iter().find(|(k, _)| k == "error").unwrap().1.as_bool();
    assert_eq!(err, Some(true));
}
