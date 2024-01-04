#[cfg(not(test))]
#[cfg(feature = "nonce")]
pub(crate) fn generate_nonce() -> Result<String, String> {
    use crate::RNG;
    use rand_chacha::rand_core::RngCore;

    let mut buf = [0u8; 10];
    RNG.with_borrow_mut(|rng| rng.as_mut().unwrap().fill_bytes(&mut buf));

    Ok(hex::encode(buf))
}

#[cfg(not(test))]
#[cfg(not(feature = "nonce"))]
pub(crate) fn generate_nonce() -> Result<String, String> {
    Ok(hex::encode("Not in use"))
}

#[cfg(test)]
pub(crate) fn generate_nonce() -> Result<String, String> {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    let mut nonce = [0u8; 10];
    rng.fill(&mut nonce);
    Ok(hex::encode(nonce))
}
