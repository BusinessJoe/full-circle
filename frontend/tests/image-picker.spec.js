/*
 * @jest-environment jsdom
 */
import ImagePicker from '../src/lib/ImagePicker.svelte';
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { fileToArrayBuffer } from '../src/lib/utils.js';

test('file and answer inputs work', async () => {
    const user = userEvent.setup();

    let image_buffer;
    let answer;

    const onSubmit = jest.fn((buf, ans) => {
        image_buffer = buf;
        answer = ans;
    });
    const { getByText, getByTestId } = render(ImagePicker, {
        onSubmit
    });

    const imageInput = getByTestId('image-input');
    const answerInput = getByTestId('answer-input');
    const submit = getByTestId('submit-button');

    // Upload a file
    let file = new File(['hello'], 'hello.png', { type: 'image/png' });
    await user.upload(imageInput, file);

    expect(imageInput.files).toHaveLength(1);
    expect(imageInput.files[0]).toStrictEqual(file);

    // Enter the answer
    await user.type(answerInput, 'greeting');

    await user.click(submit);

    await waitFor(() => {
        expect(onSubmit).toHaveBeenCalled();        
    });

    expect(image_buffer).toBeDefined();
    expect(answer).toBe('greeting');
});

test('file input required', async () => {
    const user = userEvent.setup();

    const onSubmit = jest.fn();
    const { getByText, getByTestId } = render(ImagePicker, {onSubmit});

    const imageInput = getByTestId('image-input');
    const submit = getByTestId('submit-button');

    let file = new File(['hello'], 'hello.png', { type: 'image/png' });
    await user.upload(imageInput, file);

    await user.click(submit);

    // Wait 1 second
    await new Promise(resolve => setTimeout(resolve, 1000));

    expect(onSubmit).toHaveBeenCalledTimes(0);
});

test('answer input required', async () => {
    const user = userEvent.setup();

    const onSubmit = jest.fn();
    const { getByText, getByTestId } = render(ImagePicker, {onSubmit});

    const answerInput = getByTestId('answer-input');
    const submit = getByTestId('submit-button');

    await user.type(answerInput, 'hello');
    await user.click(submit);

    // Wait 1 second
    await new Promise(resolve => setTimeout(resolve, 1000));

    expect(onSubmit).toHaveBeenCalledTimes(0);
});
