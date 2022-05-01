<template>
  <div>
    <!-- Image editing filters -->
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
        data-bs-target="#collapseFilterParams"
        aria-expanded="false"
        aria-controls="collapseFilterParams"
      >
        <i class="fa-solid fa-images"></i>
      </button>
    </p>
    <div class="collapse text-center" id="collapseFilterParams">
      <div
        class="w-1/2 block p-2 rounded-lg shadow-lg background-radial-gradient mx-auto"
      >
        <RangeInput :filter="filters.brighten" @change="filterChanged"></RangeInput>
        <RangeInput :filter="filters.huerotate" @change="filterChanged"></RangeInput>
        <RangeInput :filter="filters.blur" @change="filterChanged"></RangeInput>
        <RangeInput :filter="filters.contrast" @change="filterChanged"></RangeInput>
        <CheckboxInput :filter="filters.grayscale" @change="filterChanged"></CheckboxInput>
        <CheckboxInput :filter="filters.invert" @change="filterChanged"></CheckboxInput>
      </div>
    </div>
    </div>

    <!-- Image overlay filters -->
    <div>
      <p class="md:space-x-1 space-y-1 md:space-y-0 my-1 text-center">
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
        data-bs-target="#collapseFilterImagePreview"
        aria-expanded="false"
        aria-controls="collapseFilterImagePreview"
      >
        <i class="fa-solid fa-images"></i>
      </button>
    </p>
    <div class="collapse" id="collapseFilterImagePreview">
      <div
        class="w-1/2 block p-2 rounded-lg shadow-lg background-radial-gradient mx-auto grid lg:grid-cols-2 md:grid-cols-1 gap-2"
      >
        <div v-for="filterElement in complexFilters" :key="filterElement.id">
          <ImagePreview :filter="filterElement" @filter-image="filterImageSelected"></ImagePreview>
        </div>
      </div>
    </div>
    </div>

  </div>
</template>

<script>
import * as img from "image_process";
import RangeInput from './filters/RangeInput.vue';
import { RangeFilter, CheckboxFilter, ImagePreviewFilter } from '../utils/filters';
import CheckboxInput from "./filters/CheckboxInput.vue";
import ImagePreview from "./filters/ImagePreview.vue";

export default {
  data() {
    return {
      complexFilters: [],
      filters: {
        brighten: RangeFilter,
        huerotate: RangeFilter,
        blur: RangeFilter,
        contrast: RangeFilter,
        grayscale: CheckboxFilter,
        invert: CheckboxFilter,
        sobel: ImagePreviewFilter
      }
    }
  },
  components: { RangeInput, CheckboxInput, ImagePreview },
  created() {
    this.filters.brighten = new RangeFilter("bright", "Brighten", 0, 100, 0);
    this.filters.huerotate = new RangeFilter("hue", "Huerotate", 0, 360, 0);
    this.filters.blur = new RangeFilter("blur", "Blur", 0, 40, 0);
    this.filters.contrast = new RangeFilter("contrast", "Contrast", 0, 100, 0);
    this.filters.grayscale = new CheckboxFilter("grayscale", "Grayscale", false);
    this.filters.invert = new CheckboxFilter("invert", "Invert", false);

    this.filters.sobel = new ImagePreviewFilter("sobel", "Sobel", "/static/input/sobel.PNG", false);
    this.filters.bandsColorVertical = new ImagePreviewFilter("bandsColorVertical", "Vertical bands", "/static/input/verticalBands.PNG", false);
    this.filters.bandsColorHorizontal = new ImagePreviewFilter("bandsColorHorizontal", "Horizontal bands", "/static/input/horizontalBands.PNG", false);
    this.filters.horizontalGradient = new ImagePreviewFilter("horizontalGradient", "Horizontal gradient", "/static/input/horizontalGradient.PNG", false);
    this.filters.verticalGradient = new ImagePreviewFilter("verticalGradient", "Vertical gradient", "/static/input/verticalGradient.PNG", false);
    this.filters.verticalPixel = new ImagePreviewFilter("verticalPixel", "Vertical pixel", "/static/input/verticalPixel.PNG", false);
    this.filters.horizontalPixel = new ImagePreviewFilter("horizontalPixel", "Horizontal pixel", "/static/input/verticalPixel.PNG", false);
    this.filters.diagonalPixel = new ImagePreviewFilter("diagonalPixel", "Diagonal pixel", "/static/input/verticalPixel.PNG", false);

    this.complexFilters = [this.filters.sobel, this.filters.bandsColorVertical, this.filters.bandsColorHorizontal, this.filters.horizontalGradient,
    this.filters.verticalGradient, this.filters.verticalPixel, this.filters.horizontalPixel, this.filters.diagonalPixel];
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
      params.grayscale = JSON.parse(this.filters.grayscale.selected);
      params.invert = JSON.parse(this.filters.invert.selected);

      this.$emit("filterParamsChanged", params);
    },
    filterImageSelected(id) {
      this.complexFilters.forEach(f => f.selected = false);
      this.complexFilters.find(f => f.id == id).selected = true;

      this.$emit("filterImageChanged", id);
    }
  }
};
</script>
