<template>
  <div>
    <b-container>
      <b-card-group :key="recipe.id" v-for="recipe in recipies" columns>
        <recipe-card :recipe="recipe" @edit="onEdit" />
      </b-card-group>
    </b-container>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import { VueEditor } from "vue2-editor";

import RecipeCard from "./components/RecipeCard.vue";

import IRecipe from "./recipe-api";

@Component({
  components: {
    VueEditor,
    RecipeCard,
  },
  props: {
    recipies: Array as () => IRecipe[],
  },
})
export default class RecipeList extends Vue {
  onEdit(evt: IRecipe): void {
    console.log("Event RecipeList:", evt);
    this.$emit("edit", evt);
  }
}
</script>
