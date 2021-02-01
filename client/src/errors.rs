use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid vertex")]
    VertexInvalid,
}