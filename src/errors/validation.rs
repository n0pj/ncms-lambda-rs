use crate::errors::NcmsError;

pub const CANNOT_FIND_QUERY_STRING_PARAMETERS: NcmsError = NcmsError {
    code: "CANNOT_FIND_QUERY_STRING_PARAMETERS",
    message: "Can not find queryStringParameters parameter.",
};

pub const CANNOT_FIND_QUERY: NcmsError = NcmsError {
    code: "CANNOT_FIND_QUERY",
    message: "Can not find query parameter.",
};
