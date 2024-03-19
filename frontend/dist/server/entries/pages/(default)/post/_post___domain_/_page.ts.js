const load = async ({ params: { post, domain }, fetch }) => {
  await fetch(`/api/v1/posts/${domain}@${post}`).then((r) => r.json());
};
export {
  load
};
