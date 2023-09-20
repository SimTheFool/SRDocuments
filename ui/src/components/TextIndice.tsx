import { Theme, Text } from "@radix-ui/themes";

type TextStandard = { children?: React.ReactNode };
export const TextIndice = ({ children }: TextStandard) => {
  return (
    <Theme asChild>
      <Text
        size={"1"}
        as="span"
        style={{ display: "inline", lineHeight: 1 }}
        color="gray"
      >
        {children}
      </Text>
    </Theme>
  );
};
