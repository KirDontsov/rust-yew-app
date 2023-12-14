export class Maps {
  ymaps3;
  constructor() {
    this.config = {
        location: {
          center: [30.314997, 59.938784],
          zoom: 10
        }
      };
      this.map
  }

  async draw(element_id) {
    await ymaps3.ready;

    const { YMap, YMapDefaultSchemeLayer } = ymaps3;
  
    this.maps = new YMap(
    document.getElementById(element_id),
      this.config
    )
    this.maps.addChild(new YMapDefaultSchemeLayer());
  }
}