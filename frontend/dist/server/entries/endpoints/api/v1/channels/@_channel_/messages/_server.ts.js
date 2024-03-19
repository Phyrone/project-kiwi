import { j as json } from "../../../../../../../chunks/index.js";
import { S as SendMessageRequestSchema, r as rethink, a as respond_known_errors, G as GetRequestSearchParamsSchema } from "../../../../../../../chunks/schemas.js";
import qs from "qs";
import Joi from "joi";
import r from "rethinkdb";
function todo(description) {
  throw new Error("TODO" + (description ? ": " + description : ""));
}
const EventSchema = Joi.object({
  id: Joi.string().optional(),
  event: Joi.string().min(1).not("\n").optional(),
  data: Joi.any().required()
});
class SSEStreamImpl {
  constructor(handler) {
    this.handler = handler;
    this.controller = void 0;
  }
  //@ts-nocheck
  controller;
  onClose;
  isClosed = false;
  send(event) {
    if (this.isClosed) {
      return false;
    }
    const { error, value: validated_event } = EventSchema.validate(event, {
      cache: true,
      allowUnknown: true,
      stripUnknown: true
    });
    if (error) {
      throw new Error(`Invalid Argument: ${error.message}`);
    }
    let message = "";
    for (const [key, value] of Object.entries(validated_event)) {
      if (value === void 0)
        continue;
      if (typeof value === "string" && !value.includes("\n")) {
        message += `${key}: ${value}
`;
      } else {
        message += `${key}: ${JSON.stringify(value)}
`;
      }
    }
    message += "\n";
    this.controller.enqueue(message);
    return true;
  }
  async run() {
    await this.handler(this);
  }
  close() {
    this.isClosed = true;
    if (this.onClose) {
      this.onClose();
    }
  }
}
function sse(handler) {
  const sse2 = new SSEStreamImpl(handler);
  const stream = new ReadableStream({
    async start(controller) {
      try {
        sse2.controller = controller;
        sse2.send({
          event: "sse",
          data: "start of stream"
        });
        await sse2.run();
      } catch (e) {
        controller.error(e);
        console.error(e);
      } finally {
        if (!sse2.isClosed) {
          try {
            sse2.send({
              event: "sse",
              data: "end of stream"
            });
          } catch (e) {
          }
          sse2.close();
        }
      }
      controller.close();
    },
    cancel() {
      sse2.close();
    }
  });
  return new Response(stream, {
    headers: {
      "Content-Type": "text/event-stream",
      "Cache-Control": "no-cache",
      Connection: "keep-alive"
    },
    status: 200
  });
}
const PUT = async ({ request, params: { channel } }) => {
  try {
    const request_data_raw = await request.json();
    const request_data = await SendMessageRequestSchema.validateAsync(
      request_data_raw,
      {
        cache: true,
        allowUnknown: true
      }
    );
    const result = await r.table("message").insert({
      channel,
      //reply_to: request_data.reply_to,
      message: request_data.message,
      timestamp: r.now()
    }).run(rethink);
    return json({
      id: result.generated_keys[0]
    });
  } catch (e) {
    return respond_known_errors(e);
  }
};
const GET = async ({ url, request, params: { channel } }) => {
  const params_raw = qs.parse(url.search, { ignoreQueryPrefix: true });
  const { error, value: search } = GetRequestSearchParamsSchema.validate(params_raw, {
    cache: true,
    allowUnknown: true,
    stripUnknown: false
  });
  if (error) {
    todo("invalid query params error handling");
  }
  const channel_messages = r.table("message").filter(r.row("channel").eq(channel));
  let query = channel_messages;
  if (search.limit) {
    query = channel_messages.limit(search.limit);
  }
  if (search.offset) {
    query = channel_messages.skip(search.offset);
  }
  if (search.watch) {
    const cursor = await query.changes({
      includeInitial: true,
      squash: true,
      changefeedQueueSize: 1e5,
      includeOffsets: false,
      includeStates: true,
      includeTypes: true
    }).run(rethink);
    return sse(async (stream) => {
      try {
        while (!stream.isClosed) {
          const message = await cursor.next();
          stream.send({
            event: "message",
            data: message
          });
        }
      } finally {
        await cursor.close();
      }
    });
  } else {
    query = query.withFields("id", "message", "timestamp").orderBy(r.desc("timestamp"));
    const cursor = await query.run(rethink);
    return json({
      search,
      search_raw: url.search,
      messages: await cursor.toArray()
    });
  }
};
export {
  GET,
  PUT
};
