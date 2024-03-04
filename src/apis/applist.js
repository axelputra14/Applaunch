import axios from "axios";
import axiosTauriApiAdapter from "axios-tauri-api-adapter";
// const client = axios.create({ adapter: axiosTauriApiAdapter });

const applist = axios.create({
  adapter: axiosTauriApiAdapter,
  baseURL: "http://localhost:16850",
});

export default applist;
