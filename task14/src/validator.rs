use crate::error::AppResult;


pub fn check_port(port: u16)-> AppResult<u16>{
    if port == 0 {
        return Err("Порт должен быть больше 0".into());
    }
    Ok(port)
}