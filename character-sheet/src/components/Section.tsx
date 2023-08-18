import { Box, Heading, Section as RadSection, Text } from "@radix-ui/themes";

type SectionProps = {
  children?: React.ReactNode;
  title: string;
};

export const Section = ({ children, title }: SectionProps) => {
  return (
    <RadSection py={"0"} pb={"4"}>
      <Box py={"0"} asChild pb={"2"}>
        <Heading
          size={"4"}
          as={"h2"}
          style={{
            display: "block",
            textTransform: "uppercase",
            paddingBottom: "0",
          }}
        >
          {title}
        </Heading>
      </Box>
      {children}
    </RadSection>
  );
};
