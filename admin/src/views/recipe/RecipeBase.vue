<template>
  <div>
    <div v-if="!editing">
      <p>Recipe dude!</p>
      <b-button variant="primary" @click="onCreateNewRecipe"
        >Create New</b-button
      >
      <recipe-list :recipies="recipies" @edit="onEdit" />
    </div>
    <div v-else>
      <recipe-edit :recipe="currentRecipe" @save="onSave" @close="onClose" @delete="onDelete" />
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
import { IRecipe } from "./recipe-api";

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

  onCreateNewRecipe(): void {
    this.currentRecipe = { id: -1, title: "", description: "", content: "" };
    this.editing = true;
  }

  async onSave(evt: IRecipe): Promise<void> {
    console.log("OnSave base");
    if (evt.id > 0) await this.recipeApi.update(evt);
    else await this.recipeApi.create(evt);

    await this.loadRecipies();

    this.currentRecipe = undefined;
    this.editing = false;
  }

  async onDelete(evt: IRecipe): Promise<void> {
    await this.recipeApi.delete(evt);

    await this.loadRecipies();

    this.currentRecipe = undefined;
    this.editing = false;
  }

  onClose(): void {
    this.currentRecipe = undefined;
    this.editing = false;
  }

  loadRecipies(): void {
    this.recipeApi.getAll().then((resp) => (this.recipies = resp));
  }

  mounted(): void {
    this.loadRecipies();
  }
}
</script>
