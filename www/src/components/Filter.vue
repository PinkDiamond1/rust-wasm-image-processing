<template>
  <div>
    <p class="md:space-x-1 space-y-1 md:space-y-0 mb-1 text-center">
      <button
        class="
          inline-block
          px-6
          py-2.5
          bg-blue-600
          text-white
          font-medium
          text-xs
          leading-tight
          uppercase
          rounded
          shadow-md
          hover:bg-blue-700 hover:shadow-lg
          focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0
          active:bg-blue-800 active:shadow-lg
          transition
          duration-150
          ease-in-out
        "
        type="button"
        data-bs-toggle="collapse"
        data-bs-target="#collapseExample"
        aria-expanded="false"
        aria-controls="collapseExample"
      >
        <i class="fa-solid fa-images"></i>
      </button>
    </p>
    <div class="collapse" id="collapseExample">
      <div
        class="w-40 block p-2 rounded-lg shadow-lg background-radial-gradient"
      >
        <RangeInput :filter="filters.brighten" @change="filterChanged"></RangeInput>
        <RangeInput :filter="filters.huerotate" @change="filterChanged"></RangeInput>
        <RangeInput :filter="filters.blur" @change="filterChanged"></RangeInput>
        <RangeInput :filter="filters.contrast" @change="filterChanged"></RangeInput>
      </div>
    </div>
  </div>
</template>

<script>
import * as img from "image_process";
import RangeInput from './RangeInput.vue';
import { RangeFilter } from '../utils/filters';

const buildHuerotate = () => {
  return new RangeFilter("hue", "Huerotate", 0, 100, 0);
}

const buildBlur = () => {
  return new RangeFilter("blur", "Blur", 0, 100, 0);
}

const buildBrighten = () => {
  return new RangeFilter("bright", "Brighten", 0, 100, 0);
}

const buildContrast = () => {
  return new RangeFilter("contrast", "Contrast", 0, 100, 0);
}

export default {
  data() {
    return {
      filters: {
        brighten: RangeFilter,
        huerotate: RangeFilter,
        blur: RangeFilter,
        contrast: RangeFilter,
      }
    }
  },
  components: { RangeInput },
  created() {
    this.filters.brighten = buildBrighten();
    this.filters.huerotate = buildHuerotate();
    this.filters.blur = buildBlur();
    this.filters.contrast = buildContrast();
  },
  setup() {},

  methods: {
    filterChanged() {
      //Automatically rebuild the input parameter when one of these have changed
      let params = new img.ImageParameters();
      params.hue = parseInt(this.filters.huerotate.value);
      params.brighten = parseInt(this.filters.brighten.value);
      params.blur = parseFloat(this.filters.blur.value);
      params.constrast = parseFloat(this.filters.contrast.value);
      
      this.$emit("paramsChanged", params);
    }
  }
  // ,
  // watch: {
  //   'filters.brighten.value'(newValue) {
  //       console.log("WATCH brighten changed : " + this.filters.brighten.value);
  //   },
  //   'filters.huerotate.value'(newValue) {
  //       console.log("WATCH Huerotate changed : " + this.filters.huerotate.value);
  //   },
  //   'filters.blur.value'(newValue) {
  //       console.log("WATCH blur changed : " + this.filters.blur.value);
  //   },
  //   'filters.contrast.value'(newValue) {
  //       console.log("WATCH contrast changed : " + this.filters.contrast.value);
  //   }
  // }
};
</script>
