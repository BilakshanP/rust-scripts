```rust
enum E {
    Ex
}

fn key_num(item: &str) -> Result<(&str, i32), E> {
    match item.split_once(':') {
        Some((key, val)) => {
            match val.parse() {
                Ok(val) => Ok((key, val)),
                Err(_) => Err(E::Ex)
            }
        }

        None => Err(E::Ex)
    }
}

fn key_num_2(item: & str) -> Result<(& str, i32), E> {
    if let Some((key, val)) = item.split_once(':') {
        if let Ok(val) = val.trim().parse() {
            Ok((key, val))
        } else {
            Err(E::Ex)
        }
    } else {
        Err(E::Ex)
    }
}

fn key_num_3(item: & str) -> Result<(& str, i32), E> {
    if let Some((key, val)) = item.split_once(':') {
        if let Ok(val) = val.trim().parse() {
           return Ok((key, val));
        }
    }

    Err(E::Ex)
}

fn key_num_4(item: &str) -> Result<(&str, i32), E> {
    let Some((key, val)) = item.split_once(':') else {
        return Err(E::Ex);
    };

    let Ok(val) = val.trim().parse() else {
        return Err(E::Ex);
    };

    Ok((key, val))
}

fn key_num_5(item: &str) -> Result<(&str, i32), E> {
    let (key, val) = item.split_once(':').ok_or(E::Ex)?;
    let val = val.trim().parse().map_err(|_| E::Ex)?;

    Ok((key, val))
}

fn key_num_6(item: &str) -> Result<(&str, i32), E> {
    let Ok((key, val)) = item.split_once(':').and_then(|(key, val)| val.parse().ok().map(|val| (key, val))).ok_or(E::Ex) else {
        return Err(E::Ex);
    };

    Ok((key, val))
}

fn key_num_7(item: &str) -> Result<(&str, i32), E> {
    item.split_once(':')
        .and_then(
            |(key, val)| val.parse().ok().map(
                |val: i32| (key, val)
            )
        ).ok_or(E::Ex)
}
```