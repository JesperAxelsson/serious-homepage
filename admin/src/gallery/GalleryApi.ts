import { Dispatch, SetStateAction } from "react";
import { IAlbum } from "./models/Album";

function loadAlbumListFunc(
    setIsLoaded: Dispatch<SetStateAction<boolean>>,
    setRecipies: Dispatch<SetStateAction<IAlbum[]>>,
    setError: Dispatch<SetStateAction<any>>): () => void {

    return () => {
        fetch('http://localhost:3030/album')
            .then(res => res.json()).then(
                (result) => {
                    setIsLoaded(true);
                    console.log("Album loaded: ", result);
                    setRecipies(result);
                },
                // Note: it's important to handle errors here
                // instead of a catch() block so that we don't swallow
                // exceptions from actual bugs in components.
                (error) => {
                    setIsLoaded(true);
                    setError(error);
                })
    }
}

export { loadAlbumListFunc }