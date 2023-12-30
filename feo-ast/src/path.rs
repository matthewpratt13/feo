#[derive(Debug)]
pub struct PathExpressionSegment {
    name: Identifier,
}

#[derive(Debug)]
pub struct PathExpression {
    prefix: PathExpressionSegment,
    suffix: Vec<(Punctuation, PathExpressionSegment)>,
}
