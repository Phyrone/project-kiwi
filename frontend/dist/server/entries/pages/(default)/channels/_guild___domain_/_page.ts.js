const load = async ({ params: { guild, domain }, fetch }) => {
  return {
    props: {
      guild,
      domain
    }
  };
};
export {
  load
};
