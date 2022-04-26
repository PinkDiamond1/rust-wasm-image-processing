<template>
  <div class="container-content flex flex-col flex-1">
    <div class="rounded items-center justify-center">
      <div class="grid grid-cols-6 gap-4">
        <div class="col-start-2 col-span-4 text-center">
          <section class="mb-10 background-radial-gradient rounded-3xl">
            <div class="px-4 py-4 md:px-6 text-center lg:text-left">
              <div class="container mx-auto">
                <div class="flex items-center gap-6 justify-evenly">
                  <div class="mt-4 lg:mt-0 md:block xs:hidden">
                    <h1
                      class="
                        text-5xl
                        font-bold
                        tracking-tight
                        mb-12
                        text-gray-200
                      "
                    >
                      Upload a picture <br /><span class="text-blue-300">and customize it !</span
                      >
                    </h1>
                    <p class="text-lg text-gray-100">
                      Use filter pallet to apply edit your picture
                    </p>
                  </div>
                  <div class="mb-4 lg:mb-0">
                    <div
                      class="
                        flex
                        w-full
                        items-center
                        justify-center
                        bg-grey-lighter
                      "
                    >
                      <label
                        class="
                          w-64
                          flex flex-col
                          items-center
                          px-4
                          py-6
                          bg-white
                          text-blue
                          rounded-lg
                          shadow-lg
                          tracking-wide
                          uppercase
                          border border-blue
                          cursor-pointer
                          hover:bg-blue hover:text-sky-500
                        "
                      >
                        <svg
                          class="w-8 h-8"
                          fill="currentColor"
                          xmlns="http://www.w3.org/2000/svg"
                          viewBox="0 0 20 20"
                        >
                          <path
                            d="M16.88 9.1A4 4 0 0 1 16 17H5a5 5 0 0 1-1-9.9V7a3 3 0 0 1 4.52-2.59A4.98 4.98 0 0 1 17 8c0 .38-.04.74-.12 1.1zM11 11h3l-4-4-4 4h3v3h2v-3z"
                          />
                        </svg>
                        <span class="mt-2 text-base leading-normal"
                          >Upload a picture</span
                        >
                        <input
                          type="file"
                          class="hidden"
                          @change="handleImage"
                          accept="image/*"
                        />
                      </label>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </section>
        </div>
      </div>
    </div>
    <!-- Original uploaded image -->
    <div class="rounded items-center justify-center" v-if="image != null">
      <div class="flex flex-row">
        <div class="flex flex-1 justify-evenly p-10">
          <div class="flex justify-center">
            <img style="" :src="image" alt="" />
            <!-- <img src="../../public/static/input/block.jpg" alt="" /> -->
          </div>
        </div>

        <!-- Filters -->
        <div class="flex-2 p-2 flex flex-col items-center">
            <Filter @params-changed="onParamsChanged" />
        </div>

        <!-- Transformed image -->
        <div class="flex-1 p-10">
          <div class="flex justify-center">
            
            <div v-if="processing">
              <span>Loading...</span>
            </div>
            <div v-else>
              <img style="" :src="imageEdit" alt="" />
            </div>
            <!-- <img src="../../public/static/results/dynamic.png" alt="" /> -->
          </div>
        </div>
      </div>
    </div>
    
  </div>
</template>

<script>
import init, * as img from "image_process";
import Filter from './Filter.vue';

await init();

export default {
  components: { Filter },
  data() {
    return {
      processing: false,
      image: null,
      imageEdit: null,
    };
  },
  methods: {
    onParamsChanged(params) {
      try {
        this.processing = true;
        let image_result = img.perform_processing(this.image, params);
        this.imageEdit = "data:image/jpeg;base64," + image_result.to_base64();
      } catch (error) {
        console.log(error);
      } finally {
        this.processing = false;
      }
    },
    handleImage(e) {
      // console.log(img.hello());
      const selectedImage = e.target.files[0];
      this.createBase64Image(selectedImage);
    },
    createBase64Image(fileObject) {
      const reader = new FileReader();

      reader.onload = (e) => {
        this.image = e.target.result;
        this.onParamsChanged(null);
      };

      reader.readAsDataURL(fileObject);
    }
  },
};
</script>

<style scoped>

</style>