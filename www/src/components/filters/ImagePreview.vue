<template>
    <div>
          <label :for="filter.id" class="form-label text-sm text-blue-300 dark:text-white">{{ filter.label }}</label>
          <img :src="filter.image" alt="Preview" :class="classToogle" class="rounded-lg" @click="imageToggle"/>
    </div>
</template>

<script>
import { ImagePreviewFilter } from '../../utils/filters';

export default {
    props: {
        filter: ImagePreviewFilter
    },
    methods: {
        imageToggle(e) {
            this.filter.selected = !this.filter.selected;

            if(this.filter.selected)
                this.$emit("filter-image", this.filter.id);
        }
    },
    computed: {
        classToogle: function () {
         return this.filter.selected ? "active" : "";
        },
    },
}
</script>