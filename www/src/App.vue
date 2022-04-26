<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Header from "./components/Header.vue";
import Footer from "./components/Footer.vue";
import ImageProcessing from "./components/Image.vue";
</script>

<template>
  <div :class="mode">
    <div class="flex flex-col bg-white dark:bg-slate-900 h-screen">
      <Header @toogle-mode="toogleMode" />
      <ImageProcessing class="" />
      <Footer />
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      mode: "white",
    };
  },
  methods: {
    toogleMode(isLightMode) {
      this.mode = isLightMode ? "white" : "dark";
    },
  },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  /* text-align: center;
  color: #2c3e50;
  margin-top: 60px; */
}
</style>
