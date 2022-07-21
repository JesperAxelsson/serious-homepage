import http from 'k6/http';

import { sleep } from 'k6';


export default function () {

  http.get('http://localhost:3030/recipe/10');

  sleep(2);

}