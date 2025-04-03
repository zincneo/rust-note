la CHAPTER_ID FUNCTION_ID:
    @cargo test language::ch{{CHAPTER_ID}}::tests::ch{{CHAPTER_ID}}_{{FUNCTION_ID}} -- --nocapture

le QUESTION_ID:
    @cargo test leetcode::tests::q{{QUESTION_ID}}_testcases -- --nocapture


li CHAPTER_ID FUNCTION_ID:
    @cargo test lists::tests::ch{{CHAPTER_ID}}_{{FUNCTION_ID}} -- --nocapture

serve:
    rm -rf target/doc
    @cargo doc
    cp -r images/ target/doc
    miniserve target/doc
