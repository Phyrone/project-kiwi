const load = async ({ params: { guild, channel, domain }, fetch }) => {
  return {
    props: {
      guild,
      channel,
      domain
    }
  };
};
export {
  load
};
