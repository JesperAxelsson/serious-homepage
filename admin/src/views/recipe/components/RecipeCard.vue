<template>
  <b-card
    :title="recipe.title"
    :sub-title="recipe.description"
    style="max-width: 20rem"
  >
    <b-button variant="primary" @click="onClick">Edit</b-button>
  </b-card>
</template>

<script lang="ts">
import { Component, Vue, Prop } from "vue-property-decorator";
import { VueEditor } from "vue2-editor";
import IRecipe from "./../recipe-api";

@Component({
  components: {
    VueEditor,
  },
})
export default class RecipeCard extends Vue {
  @Prop(Object) readonly recipe: IRecipe | undefined;

  onClick() : void {
    console.log("Event Edit:", this.recipe);
    this.$emit("edit", this.recipe);
  }
}
</script>
