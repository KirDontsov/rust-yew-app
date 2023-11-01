export class PieChart {
  chart;
  constructor() {
    let data = {
      labels: [
        'Просроченные',
        'Закрытые',
        'Новые'
      ],
      datasets: [{
        label: 'My First Dataset',
        data: [2, 20, 15],
        backgroundColor: [
          'rgb(255, 99, 132)',
          'rgb(54, 162, 235)',
          'rgb(255, 205, 86)'
        ],
        hoverOffset: 4
      }]
    };

    this.config = {
      type: 'doughnut',
      data: data,
    };
  }

  draw(element_id) {
    this.chart = new Chart(
      document.getElementById(element_id),
      this.config
    )
  }
}