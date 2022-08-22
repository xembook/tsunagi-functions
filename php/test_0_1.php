<?php

include 'tsunagi-sdk-0.1.php';

class test_0_1 extends \PHPUnit\Framework\TestCase {

    public function testWithTaxAndTip() {
        $meal = 100;
        $tax = 10;
        $tip = 20;
        $result = restaurant_check($meal, $tax, $tip);
        $this->assertEquals(130, $result);
    }

}
?>


