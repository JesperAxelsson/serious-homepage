<template>
  <div class="home">
    <p>Recipe dude!</p>
    <div :key="recipe.id" v-for="recipe in recipies">
      <recipe-card :recipe="recipe" />
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import { VueEditor } from "vue2-editor";

import RecipeCard from "./components/RecipeCard.vue";

import RecipeApi from "./recipeApi";
import IRecipe from "./recipeApi";

@Component({
  components: {
    VueEditor,RecipeCard
  },
})
export default class Recipe extends Vue {
  recipeApi: RecipeApi = new RecipeApi();
  content = "";
  recipies: IRecipe[] = [];

  mounted(): void {
    this.recipeApi.getAll().then((resp) => (this.recipies = resp));
    // axios
    //   .get('https://api.coindesk.com/v1/bpi/currentprice.json')
    //   .then(response => (this.info = response))
  }
}
</script>
