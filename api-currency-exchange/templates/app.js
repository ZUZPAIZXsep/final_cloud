// ดึงข้อมูลจาก API และแสดงผลบนหน้าเว็บ
function getExchangeRates() {
     $.get("https://private-810151-exchangeusdapi.apiary-mock.com/currency_rate", function(data) {
       var rates = data[0].rates;
       var currencies = Object.keys(rates);
       
       // แสดงจำนวนเงินในกระเป๋าเป็น USD
       var balance = 100000;
       $("#balance").text("$" + balance.toFixed(2));
       
       // แสดงรายการอัตราการแลกเปลี่ยนและปุ่มเปลี่ยนเงิน
       var rows = "";
       for (var i = 0; i < currencies.length; i++) {
         var currency = currencies[i];
         var rate = rates[currency];
         rows += "<tr><td>" + currency + "</td><td>" + rate.toFixed(3) + "</td><td><button class='btn btn-primary exchange-btn' data-currency='" + currency + "'>Exchange</button></td></tr>";
       }
       $("#rates").html(rows);
       
       // กำหนดเหตุการณ์คลิกปุ่มเปลี่ยนเงิน
       $(".exchange-btn").click(function() {
         var currency = $(this).data("currency");
         var amount = prompt("Enter amount to exchange:");
         if (amount && amount > 0) {
           $.post("https://private-810151-exchangeusdapi.apiary-mock.com/USD/to/" + currency, {from_currency: "USD", to_currency: currency, amount: amount}, function(data) {
             var convertedAmount = data.converted_amount;
             balance -= amount;
             $("#balance").text("$" + balance.toFixed(2));
             alert("You exchanged " + amount + " USD to " + convertedAmount.toFixed(2) + " " + currency + ".");
           }).fail(function() {
             alert("Exchange failed. Please try again.");
           });
         }
       });
     }).fail(function() {
       alert("Failed to load exchange rates. Please try again.");
     });
   }
   
   // เรียกใช้งานฟังก์ชัน getExchangeRates() เมื่อหน้าเว็บโหลดเสร็จ
   $(document).ready(function() {
     getExchangeRates();
   });
   