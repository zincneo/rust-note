basic CHAPTER_ID FUNCTION_ID:
    @cargo test basic::ch{{CHAPTER_ID}}::tests::ch{{CHAPTER_ID}}_{{FUNCTION_ID}} -- --nocapture

advance CHAPTER_ID FUNCTION_ID:
    @cargo test advance::ch{{CHAPTER_ID}}::tests::ch{{CHAPTER_ID}}_{{FUNCTION_ID}} -- --nocapture
