use crate::common::*;

#[test]
fn sum_both() {
    let expr = "<math>
        <munderover>
            <mo>∑</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </munderover>
        <mi>n</mi>
    </math>";
    test("zh", "SimpleSpeak", expr, "和 從 n 等於 1 到 10 項目 n");
}

#[test]
fn sum_under() {
    let expr = "<math>
        <munder>
            <mo>∑</mo>
            <mi>S</mi>
        </munder>
        <mi>i</mi>
    </math>";
    test("zh", "SimpleSpeak", expr, "和 下層 大寫 s 項目 i");
}
#[test]
fn sum_both_msubsup() {
    let expr = "<math>
        <msubsup>
            <mo>∑</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </msubsup>
        <mi>n</mi>
    </math>";
    test("zh", "SimpleSpeak", expr, "和 從 n 等於 1 到 10 項目 n");
}

#[test]
fn sum_sub() {
    let expr = "<math>
        <msub>
            <mo>∑</mo>
            <mi>S</mi>
        </msub>
        <mi>i</mi>
    </math>";
    test("zh", "SimpleSpeak", expr, "和 下層 大寫 s 項目 i");
}

#[test]
fn sum() {
    let expr = "<math>
            <mo>∑</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("zh", "SimpleSpeak", expr, "和 項目 a 下標 i");
}

#[test]
fn product_both() {
    let expr = "<math>
        <munderover>
            <mo>∏</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </munderover>
        <mi>n</mi>
    </math>";
    test("zh", "SimpleSpeak", expr, "積 從 n 等於 1 到 10 項目 n");
}

#[test]
fn product_under() {
    let expr = "<math>
        <munder>
            <mo>∏</mo>
            <mi>S</mi>
        </munder>
        <mi>i</mi>
    </math>";
    test("zh", "SimpleSpeak", expr, "積 下層 大寫 s 項目 i");
}

#[test]
fn product() {
    let expr = "<math>
            <mo>∏</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("zh", "SimpleSpeak", expr, "積 項目 a 下標 i");
}

#[test]
fn intersection_both() {
    let expr = "<math>
        <munderover>
            <mo>⋂</mo>
            <mrow><mi>i</mi><mo>=</mo><mn>1</mn> </mrow>
            <mn>10</mn>
        </munderover>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("zh", "SimpleSpeak", expr, "交集 從 i 等於 1 到 10 項目; 大寫 s 下標 i");
}

#[test]
fn intersection_under() {
    let expr = "<math>
        <munder>
            <mo>⋂</mo>
            <mi>C</mi>
        </munder>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("zh", "SimpleSpeak", expr, "交集 下層 大寫 c 項目, 大寫 s 下標 i");
}

#[test]
fn intersection() {
    let expr = "<math>
            <mo>⋂</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("zh", "SimpleSpeak", expr, "交集 項目 大寫 s 下標 i");
}

#[test]
fn union_both() {
    let expr = "<math>
        <munderover>
            <mo>⋃</mo>
            <mrow><mi>i</mi><mo>=</mo><mn>1</mn> </mrow>
            <mn>10</mn>
        </munderover>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("zh", "SimpleSpeak", expr, "聯集 從 i 等於 1 到 10 項目; 大寫 s 下標 i");
}

#[test]
fn union_under() {
    let expr = "<math>
        <munder>
            <mo>⋃</mo>
            <mi>C</mi>
        </munder>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("zh", "SimpleSpeak", expr, "聯集 下層 大寫 c 項目, 大寫 s 下標 i");
}

#[test]
fn union() {
    let expr = "<math>
            <mo>⋃</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("zh", "SimpleSpeak", expr, "聯集 項目 大寫 s 下標 i");
}

#[test]
fn integral_both() {
    let expr = "<math>
            <mrow>
                <msubsup>
                    <mo>∫</mo>
                    <mn>0</mn>
                    <mn>1</mn>
                </msubsup>
                <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            </mrow>
            <mtext>&#x2009;</mtext><mi>d</mi><mi>x</mi>
        </math>";
    test("zh", "SimpleSpeak", expr, "積分 從 0 到 1 項目, f x; d x");
}

#[test]
fn integral_under() {
    let expr = "<math>
        <munder>
            <mo>∫</mo>
            <mi>ℝ</mi>
        </munder>
        <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
        <mi>d</mi><mi>x</mi>
        </math>";
    test("zh", "SimpleSpeak", expr, "積分 下層 實數集 項目; f x d x");
}

#[test]
fn integral() {
    let expr = "<math>
            <mo>∫</mo>
            <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            <mi>d</mi><mi>x</mi>
            </math>";
    test("zh", "SimpleSpeak", expr, "積分 項目 f x d x");
}