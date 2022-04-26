<template>
  <div class="text-right p-8">
    <span class="cursor-pointer hover:text-sky-500 dark:text-white" @click="toogleMode">
      <i :class="classToogle" class="text-lg" aria-hidden="true"></i>
    </span>
  </div>
</template>

<script>
export default {
  data() {
    return {
      lightMode: true,
    };
  },
  emits: ["toogleMode"],
  methods: {
    toogleMode() {
      this.lightMode = !this.lightMode;
      this.$emit("toogleMode", this.lightMode);
    },
  },
  computed: {
    classToogle: function () {
      return this.lightMode ? "fa-solid fa-moon" : "fa-solid fa-sun";
    },
  },
};
</script>