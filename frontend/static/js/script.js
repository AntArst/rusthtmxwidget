document.addEventListener('DOMContentLoaded', function() {

  // Listen to HTMX events
  ['htmx:configRequest', 'htmx:afterOnLoad', 'htmx:afterSwap'].forEach(eventType => {
    document.body.addEventListener(eventType, function(event) {
      console.log(eventType, event.detail);
    });
  });

  // Update the Units label based on the operation type
  const units_label = document.getElementById('units_label');
  const updateUnitsLabel = (operation_type) => {
    if (operation_type === 'buy') {
      units_label.textContent = "Total Dollar Amount:";
    } else if (operation_type === 'sell') {
      units_label.textContent = "Total Number of Coins:";
    }
  };

  // Handle Buy/Sell button clicks
  const buy_button = document.getElementById('buy_button');
  const sell_button = document.getElementById('sell_button');
  const operation_type_input = document.getElementById('operation_type');
  
  const setActiveButton = (active_button, inactive_button) => {
    active_button.classList.add('active');
    inactive_button.classList.remove('active');
  };

  buy_button.addEventListener('click', function() {
    operation_type_input.value = 'buy';
    setActiveButton(buy_button, sell_button);
    updateUnitsLabel('buy');
  });

  sell_button.addEventListener('click', function() {
    operation_type_input.value = 'sell';
    setActiveButton(sell_button, buy_button);
    updateUnitsLabel('sell');
  });

  // Handle sliders
  const high_percent_slider = document.getElementById('high_percent');
  const high_percent_value = document.getElementById('high_percent_value');
  const low_percent_slider = document.getElementById('low_percent');
  const low_percent_value = document.getElementById('low_percent_value');

  // Initialize read-only fields with current slider values
  high_percent_value.value = high_percent_slider.value;
  low_percent_value.value = low_percent_slider.value;

  const updateSliderValue = (high_slider, low_slider, high_output, low_output) => {
    high_slider.addEventListener('input', function() {
      high_output.value = high_slider.value;

      // Ensure Low Percent is always less than High Percent
      if (parseInt(low_slider.value, 10) >= parseInt(high_slider.value, 10)) {
        low_slider.value = parseInt(high_slider.value, 10) - 1;
        low_output.value = low_slider.value;
      }
    });

    low_slider.addEventListener('input', function() {
      low_output.value = low_slider.value;

      // Ensure Low Percent is always less than High Percent
      if (parseInt(low_slider.value, 10) >= parseInt(high_slider.value, 10)) {
        low_slider.value = parseInt(high_slider.value, 10) - 1;
        low_output.value = low_slider.value;
      }
    });
  };

  // Attach event listeners to sliders
  updateSliderValue(high_percent_slider, low_percent_slider, high_percent_value, low_percent_value);
});
