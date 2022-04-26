<template>
    <div class="relative pt-1">
          <label :for="filter.id" class="form-label text-blue-300 dark:text-white">{{ filter.label }}</label>
          <input
            type="range"
            class="
              form-range
              appearance-none
              w-full
              h-6
              p-0
              bg-transparent
              focus:outline-none focus:ring-0 focus:shadow-none
            "
            :id="filter.id"
            :min="filter.min"
            :max="filter.max"
            v-model="filter.value"
          />
    </div>
</template>

<script>
import { RangeFilter } from '../utils/filters';

export default {
    props: {
        filter: RangeFilter
    },
    created() {
      console.log(JSON.parse(JSON.stringify(this.filter)));
    }
}
</script>