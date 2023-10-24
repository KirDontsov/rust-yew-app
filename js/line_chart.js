export class LineChart {
  chart;
  constructor() {
    let data = {
      labels: ["янв", "фев", "мар", "апр", "май", "июн", "июл", "авг", "сен", "окт", "ноя", "дек"],
      datasets: [{
        label: 'Выручка/год',
        backgroundColor: 'rgb(255, 99, 132)',
        borderColor: 'rgb(255, 99, 132)',
        data: [0, 250_000, 300_000, 400_000, 450_000, 550_000, 630_000, 740_000, 690_000, 790_000, 950_000, 0],
      }]
    };

    this.config = {
      type: 'line',
      data: data,
      options: {
        responsive: false,
        scales: {
          y: {
            suggestedMin: 0,
            suggestedMax: 50
          }
        }
      }
    };
  }

  draw(element_id) {
    this.chart = new Chart(
      document.getElementById(element_id),
      this.config
    )
  }
}