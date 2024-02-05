pub fn convert<T: Into<String>>(input: T) -> String {
    betacode::converter::convert(input)
}
