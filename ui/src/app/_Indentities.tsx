import { Card } from "@/components/Card";
import { FlexList } from "@/components/FlexList";
import { Section } from "@/components/Section";
import { TextStandard } from "@/components/TextStandard";
import { TitleMin } from "@/components/TitleMin";
import { Box, Flex } from "@radix-ui/themes";
import { Character, Identity as CharIdentity } from "resources";
import { capitalize } from "@/utils/capitalize";
import { TitleSection } from "@/components/TitleSection";

type IdentitiesProps = {
  char: Character;
};

export const Identities = ({ char }: IdentitiesProps) => {
  return (
    <Section title={<TitleSection>Identités</TitleSection>}>
      {char.identities?.map((i) => (
        <>
          <Identity identity={i} />
        </>
      ))}
    </Section>
  );
};

const Identity = ({
  identity: {
    contacts,
    description,
    licences,
    lifestyle,
    name,
    nuyens,
    price,
    quality,
  },
}: {
  identity: CharIdentity;
}) => {
  const qualityStr = quality ? `${quality} - ${price}¥` : null;
  const lifestyleStr = lifestyle
    ? `${lifestyle?.name} - ${lifestyle?.price}¥`
    : null;

  return (
    <Flex wrap={"wrap"} align={"stretch"}>
      {(lifestyle || quality) && (
        <Container width={"90%"}>
          <Card>
            <Box>
              <TitleMin
                title={name && capitalize(name)}
                subtitle={[qualityStr, lifestyleStr]
                  .filter((x) => x)
                  .join(" - ")}
              />
              <TextStandard>{description}</TextStandard>
            </Box>
          </Card>
        </Container>
      )}
      {quality && (
        <Container width={"50%"}>
          <Card title={"nuyens"}>
            <Flex justify={"between"} align={"end"} height={"100%"}>
              <TextStandard>_</TextStandard>
              <TextStandard>{nuyens ? `/${nuyens}¥` : null}</TextStandard>
            </Flex>
          </Card>
        </Container>
      )}
      {licences?.map((l) => (
        <Container width={"50%"}>
          <Card title={"licence"}>
            <TextStandard>{l.name}</TextStandard>
            <TitleMin subtitle={`${l.quality}-${l.price}¥`} />
          </Card>
        </Container>
      ))}
      {contacts?.map((c) => {
        return (
          <Container width={"50%"}>
            <Card title={"contact"}>
              <TitleMin
                title={c.name}
                subtitle={`L${c.loyalty}-R${c.connection}`}
              />
              <TextStandard>{c.description}</TextStandard>
            </Card>
          </Container>
        );
      })}
    </Flex>
  );
};

const Container = ({
  children,
  width,
}: {
  children?: React.ReactNode;
  width?: string;
}) => {
  return (
    <Box
      pr={"2"}
      grow={"1"}
      pb={"2"}
      style={{
        width,
      }}
    >
      {children}
    </Box>
  );
};
