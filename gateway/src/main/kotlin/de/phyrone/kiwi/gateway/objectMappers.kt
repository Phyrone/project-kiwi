package de.phyrone.kiwi.gateway

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.cbor.databind.CBORMapper
import com.fasterxml.jackson.dataformat.csv.CsvMapper
import com.fasterxml.jackson.dataformat.protobuf.ProtobufMapper
import com.fasterxml.jackson.dataformat.xml.XmlMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLMapper
import org.koin.core.annotation.Single
import org.msgpack.jackson.dataformat.MessagePackMapper

object ObjectMappers {
    private const val PREFIX = "objectMapper"
    const val JSON = "$PREFIX.json"
    const val XML = "$PREFIX.xml"
}


@Single
fun jsonObjectMapper() = ObjectMapper().setup()

@Single
fun xmlObjectMapper() = XmlMapper().setup()

@Single
fun cborObjectMapper() = CBORMapper().setup()

@Single
fun protobufObjectMapper() = ProtobufMapper().setup()

@Single
fun msgpackMapper() = MessagePackMapper().setup()

@Single
fun yamlObjectMapper() = YAMLMapper().setup()

@Single
fun csvObjectMapper() = CsvMapper().setup()