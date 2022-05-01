<template>
    <div class="relative pt-1">
          <label :for="filter.id" class="form-label text-blue-300 dark:text-white">{{ filter.label }}</label>
          <input
            type="checkbox"
            class="
              ml-2
            "
            :id="filter.id"
            v-model="filter.selected"
          />
    </div>
</template>

<script>
import { CheckboxFilter } from '../../utils/filters';

export default {
    props: {
        filter: CheckboxFilter
    }
}
</script>