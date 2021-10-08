<template>
  <div class="home">
    <p>Title: {{ recipe.Title }}</p>
    <p>Description: {{ recipe.description }}</p>
    <div style="max-width: 500px; max-height: 500px;">
      <vue-editor v-model="content" style="max-width: 500px; max-height: 500px;"/>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import { VueEditor } from "vue2-editor";
import IRecipe from "./../recipeApi";

@Component({
  components: {
    VueEditor,
  },
  props: {
    recipe: Object as () => IRecipe,
  },
})
export default class RecipeCard extends Vue {
  content=""
}
</script>
