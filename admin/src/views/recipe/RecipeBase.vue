<template>
  <div>
    <p>Recipe dude!</p>
    <div v-if="!editing">
      <recipe-list :recipies="recipies" @edit="onEdit" />
    </div>
    <div v-else>
      <recipe-edit :recipe="currentRecipe" @save="onSave" @close="onClose" />
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import { VueEditor } from "vue2-editor";

import RecipeCard from "./components/RecipeCard.vue";
import RecipeList from "./RecipeList.vue";
import RecipeEdit from "./RecipeEdit.vue";

import RecipeApi from "./recipe-api";
import IRecipe from "./recipe-api";

@Component({
  components: {
    VueEditor,
    RecipeCard,
    RecipeList,
    RecipeEdit,
  },
})
export default class RecipeBase extends Vue {
  editing = false;
  recipeApi: RecipeApi = new RecipeApi();
  content = "";
  recipies: IRecipe[] = [];
  currentRecipe: IRecipe | undefined;

onEdit(evt: IRecipe): void {
    this.currentRecipe = evt;
    this.editing = true;
  }

  async onSave(evt: IRecipe) :  Promise<void>{
    console.log("OnSave base")
    await this.recipeApi.update(evt);

    this.currentRecipe = undefined;
    this.editing = false;
  }

  onClose() : void{
    this.currentRecipe = undefined;
    this.editing = false;
  }

  loadRecipies() : void{
    this.recipeApi.getAll().then((resp) => (this.recipies = resp));
  }

  mounted(): void {
    this.loadRecipies();
  }
}
</script>
