import { Dispatch, SetStateAction } from "react";

function login(
    setIsLoaded: Dispatch<SetStateAction<boolean>>,
    setLoggedIn: Dispatch<SetStateAction<any>>,
    setError: Dispatch<SetStateAction<any>>): () => void {

    return () => {
        fetch('http://localhost:3030/album')
            .then(res => res.json()).then(
                (result) => {
                    setIsLoaded(true);
                    console.log("User logged in: ", result);
                    setLoggedIn(result);
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

export { login }