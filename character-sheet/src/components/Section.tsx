import {
  Box,
  Flex,
  Heading,
  Section as RadSection,
  Text,
} from "@radix-ui/themes";

type SectionProps = {
  children?: React.ReactNode;
  title: string;
  separator?: "left";
};

export const Section = ({ children, title, separator }: SectionProps) => {
  return (
    <RadSection py={"0"} mb={"3"}>
      <Box pt={"0"} asChild pb={"1"}>
        <Heading
          size={"4"}
          as={"h2"}
          style={{
            display: "block",
            textTransform: "uppercase",
          }}
        >
          {title}
        </Heading>
      </Box>
      {children}
    </RadSection>
  );
};
